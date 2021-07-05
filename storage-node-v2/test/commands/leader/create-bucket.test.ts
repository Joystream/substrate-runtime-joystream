import { assert } from 'chai'
import * as child from 'child_process';

class CommandBuilder {
  private executable: string = 'yarn storage-node'
  private command: string
  private arguments: string

  constructor (command: string = '', args: string = ''){
    this.command = command
    this.arguments = args
  }

  args(args: string): CommandBuilder {
    this.arguments = args
    return this
  }

  cmd(cmd: string): CommandBuilder {
    this.command = cmd
    return this
  }

  final(): string {
    return [this.executable, this.command, this.arguments].join(' ')
  }

}

describe('leader:create-bucket', () => {
  before(function(done) {
    this.timeout(60000)
    const cmd =  new CommandBuilder().cmd('dev:init').final()
    console.log('dev:init started.')
    child.exec(cmd,(error: any, stdout: string, stderr: string) => {
      console.log("Initialized.")
      done()
    });
  });

  const cmd =  new CommandBuilder().cmd('leader:create-bucket').args('').final()
  describe(cmd, () => {
    it("fails without keyfile", function(done) {
      child.exec(cmd, {timeout: 5000}, (error: any, stdout: string, stderr: string) => {
          assert(stderr.includes('Key'), 'Should contain an error.')

          done()
      });
    });
  })  
  
  const cmd2 =  new CommandBuilder().cmd('leader:create-bucket').args('--dev').final()
  describe(cmd2, () => {

    it("fails without keyfile", function(done) {
      this.timeout(60000)
      child.exec(cmd2, {timeout: 9000}, (error: any, stdout: string, stderr: string) => {
          assert(stdout.includes('Extrinsic successful!'), 'Should be successful extrinsic')

          done()
      })
    })
  })
})






