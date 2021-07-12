## Setup

### Configuring the AWS CLI
We’re going to use the AWS CLI to access AWS resources from the command line. 

Follow [the official directions](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-install.html) for your system.

Once the AWS CLI is installed, configure a profile

`aws configure --profile joystream-user`

### Create a key pair
Change profile and region parameters according to your configuration
```
aws ec2 create-key-pair --key-name joystream-key --profile joystream-user --region us-east-1 --query 'KeyMaterial' --output text > joystream-key.pem
```

Set the permissions for the key pair 

`chmod 400 joystream-key.pem`

### Install Ansible
On Mac run the command:
* `brew install ansible`

Follow [the official installation guide](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html) for your system.

# How to run
Copy and edit the file `deploy-config.sample.cfg` and update parameters like AWS_KEY_PAIR_NAME, KEY_PATH
Run the `deploy-infra.sh` script to deploy the infrastructure

```
cd devops/infrastructure
./deploy-infra.sh your-deploy-config.cfg
```
