import * as awsx from '@pulumi/awsx'
import * as eks from '@pulumi/eks'
// import * as k8s from '@pulumi/kubernetes'
import * as docker from '@pulumi/docker'
import * as pulumi from '@pulumi/pulumi'
// import * as k8sjs from './k8sjs'
import * as k8s from '@pulumi/kubernetes'
import * as fs from 'fs'

require('dotenv').config()

// const awsConfig = new pulumi.Config('aws')

// // Create a VPC for our cluster.
// const vpc = new awsx.ec2.Vpc('vpc', { numberOfAvailabilityZones: 2 })

// // Create an EKS cluster with the default configuration.
// const cluster = new eks.Cluster('eksctl-my-cluster', {
//   vpcId: vpc.id,
//   subnetIds: vpc.publicSubnetIds,
//   instanceType: 't2.large',
//   providerCredentialOpts: {
//     profileName: awsConfig.get('profile'),
//   },
// })

// // Export the cluster's kubeconfig.
// export const kubeconfig = cluster.kubeconfig

// // Create a repository
// const repo = new awsx.ecr.Repository('joystream/apps')

// export const joystreamAppsImage = repo.buildAndPushImage({
//   dockerfile: '../../../apps.Dockerfile',
//   context: '../../../',
// })

// Create image from local app
// export const joystreamAppsImage = new docker.Image('joystream/apps', {
//   build: {
//     context: '../../../',
//     dockerfile: '../../../apps.Dockerfile',
//   },
//   imageName: 'joystream/apps:latest',
//   skipPush: true,
// }).baseImageName

export const joystreamAppsImage = 'joystream/apps'

const name = 'query-node'

// Create a Kubernetes Namespace
// const ns = new k8s.core.v1.Namespace(name, {}, { provider: cluster.provider })
const ns = new k8s.core.v1.Namespace(name, {})

// Export the Namespace name
export const namespaceName = ns.metadata.name

const appLabels = { appClass: name }

const defsConfig = new k8s.core.v1.ConfigMap(name, {
  metadata: { namespace: namespaceName, labels: appLabels },
  data: { 'fileData': fs.readFileSync('../../../types/augment/all/defs.json').toString() },
})
const defsConfigName = defsConfig.metadata.apply((m) => m.name)

// Create a Deployment

const databaseLabels = { app: 'postgres-db' }
const databaseDeployment = new k8s.apps.v1.Deployment('postgres-db', {
  metadata: {
    namespace: namespaceName,
    labels: databaseLabels,
  },
  spec: {
    selector: { matchLabels: databaseLabels },
    template: {
      metadata: { labels: databaseLabels },
      spec: {
        containers: [
          {
            name: 'postgres-db',
            image: 'postgres:12',
            env: [
              { name: 'POSTGRES_USER', value: process.env.DB_USER! },
              { name: 'POSTGRES_PASSWORD', value: process.env.DB_PASS! },
              { name: 'POSTGRES_DB', value: process.env.INDEXER_DB_NAME! },
            ],
            ports: [{ containerPort: 5432 }],
          },
        ],
      },
    },
  },
})

const databaseService = new k8s.core.v1.Service('postgres-db', {
  metadata: {
    namespace: namespaceName,
    labels: databaseDeployment.metadata.labels,
    name: 'postgres-db',
  },
  spec: {
    ports: [{ port: 5432 }],
    selector: databaseDeployment.spec.template.metadata.labels,
  },
})

// Create an example Job.
const exampleJob = new k8s.batch.v1.Job(
  'db-migration',
  {
    metadata: {
      namespace: namespaceName,
    },
    spec: {
      backoffLimit: 0,
      template: {
        spec: {
          containers: [
            {
              name: 'db-migration',
              image: joystreamAppsImage,
              imagePullPolicy: 'IfNotPresent',
              resources: { requests: { cpu: '100m', memory: '100Mi' } },
              env: [
                {
                  name: 'WARTHOG_DB_HOST',
                  value: 'postgres-db',
                },
                {
                  name: 'DB_HOST',
                  value: 'postgres-db',
                },
                { name: 'DB_NAME', value: process.env.DB_NAME! },
                { name: 'DB_PASS', value: process.env.DB_PASS! },
              ],
              command: ['/bin/sh', '-c'],
              args: ['yarn workspace query-node-root db:prepare; yarn workspace query-node-root db:migrate'],
            },
          ],
          restartPolicy: 'Never',
        },
      },
    },
  },
  { dependsOn: databaseService }
  // { provider: provider }
)

const deployment = new k8s.apps.v1.Deployment(
  name,
  {
    metadata: {
      namespace: namespaceName,
      labels: appLabels,
    },
    spec: {
      replicas: 1,
      selector: { matchLabels: appLabels },
      template: {
        metadata: {
          labels: appLabels,
        },
        spec: {
          containers: [
            {
              name: 'redis',
              image: 'redis:6.0-alpine',
              ports: [{ containerPort: 6379 }],
            },
            {
              name: 'indexer',
              image: 'joystream/hydra-indexer:2.1.0-beta.9',
              env: [
                // ...envCopy,
                { name: 'DB_HOST', value: 'postgres-db' },
                { name: 'DB_NAME', value: process.env.INDEXER_DB_NAME! },
                { name: 'DB_PASS', value: process.env.DB_PASS! },
                { name: 'INDEXER_WORKERS', value: '5' },
                { name: 'REDIS_URI', value: 'redis://localhost:6379/0' },
                { name: 'DEBUG', value: 'index-builder:*' },
                { name: 'WS_PROVIDER_ENDPOINT_URI', value: process.env.WS_PROVIDER_ENDPOINT_URI! },
                { name: 'TYPES_JSON', value: 'types.json' },
                { name: 'PGUSER', value: process.env.DB_USER! },
                { name: 'BLOCK_HEIGHT', value: process.env.BLOCK_HEIGHT! },
              ],
              volumeMounts: [
                {
                  mountPath: '/home/hydra/packages/hydra-indexer/types.json',
                  name: 'indexer-volume',
                  subPath: 'fileData',
                },
              ],
              command: ['/bin/sh', '-c'],
              args: ['yarn db:bootstrap && yarn start:prod'],
            },
            {
              name: 'hydra-indexer-gateway',
              image: 'joystream/hydra-indexer-gateway:2.1.0-beta.5',
              env: [
                { name: 'WARTHOG_STARTER_DB_DATABASE', value: process.env.INDEXER_DB_NAME! },
                { name: 'WARTHOG_STARTER_DB_HOST', value: 'postgres-db' },
                { name: 'WARTHOG_STARTER_DB_PASSWORD', value: process.env.DB_PASS! },
                { name: 'WARTHOG_STARTER_DB_PORT', value: process.env.DB_PORT! },
                { name: 'WARTHOG_STARTER_DB_USERNAME', value: process.env.DB_USER! },
                { name: 'WARTHOG_STARTER_REDIS_URI', value: 'redis://localhost:6379/0' },
                { name: 'WARTHOG_APP_PORT', value: process.env.WARTHOG_APP_PORT! },
                { name: 'PORT', value: process.env.WARTHOG_APP_PORT! },
                { name: 'DEBUG', value: '*' },
              ],
              ports: [{ containerPort: 4002 }],
            },
            {
              name: 'processor',
              image: joystreamAppsImage,
              imagePullPolicy: 'IfNotPresent',
              env: [
                {
                  name: 'INDEXER_ENDPOINT_URL',
                  value: `http://localhost:${process.env.WARTHOG_APP_PORT}/graphql`,
                },
                { name: 'TYPEORM_HOST', value: 'postgres-db' },
                { name: 'TYPEORM_DATABASE', value: process.env.DB_NAME! },
                { name: 'DEBUG', value: 'index-builder:*' },
                { name: 'PROCESSOR_POLL_INTERVAL', value: '1000' },
              ],
              volumeMounts: [
                {
                  mountPath: '/joystream/query-node/mappings/lib/generated/types/typedefs.json',
                  name: 'processor-volume',
                  subPath: 'fileData',
                },
              ],
              args: ['workspace', 'query-node-root', 'processor:start'],
            },
            {
              name: 'graphql-server',
              image: joystreamAppsImage,
              imagePullPolicy: 'IfNotPresent',
              env: [
                { name: 'DB_HOST', value: 'postgres-db' },
                { name: 'DB_PASS', value: process.env.DB_PASS! },
                { name: 'DB_USER', value: process.env.DB_USER! },
                { name: 'DB_PORT', value: process.env.DB_PORT! },
                { name: 'DB_NAME', value: process.env.DB_NAME! },
                { name: 'GRAPHQL_SERVER_HOST', value: process.env.GRAPHQL_SERVER_HOST! },
                { name: 'GRAPHQL_SERVER_PORT', value: process.env.GRAPHQL_SERVER_PORT! },
              ],
              ports: [{ name: 'graph-ql-port', containerPort: Number(process.env.GRAPHQL_SERVER_PORT!) }],
              args: ['workspace', 'query-node-root', 'query-node:start:prod'],
            },
          ],
          volumes: [
            {
              name: 'processor-volume',
              configMap: {
                name: defsConfigName,
              },
            },
            {
              name: 'indexer-volume',
              configMap: {
                name: defsConfigName,
              },
            },
          ],
        },
      },
    },
  },
  { dependsOn: exampleJob }
  // {
  //   provider: cluster.provider,
  // }
)

// Export the Deployment name
export const deploymentName = deployment.metadata.name

// Create a LoadBalancer Service for the NGINX Deployment
const service = new k8s.core.v1.Service(
  name,
  {
    metadata: {
      labels: appLabels,
      namespace: namespaceName,
    },
    spec: {
      type: 'NodePort',
      ports: [
        { name: 'port-1', port: 8081, targetPort: 'graph-ql-port' },
        { name: 'port-2', port: 4000, targetPort: 4002 },
      ],
      selector: appLabels,
    },
  }
  // {
  //   provider: cluster.provider,
  // }
)

// Export the Service name and public LoadBalancer Endpoint
export const serviceName = service.metadata.name

// When "done", this will print the public IP.
export let serviceHostname: pulumi.Output<string>

serviceHostname = service.status.loadBalancer.ingress[0].hostname
