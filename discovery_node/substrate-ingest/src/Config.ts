import { config } from 'dotenv';
import * as fs  from 'fs'
import * as path from 'path'
import * as yaml from 'js-yaml'
import log4js = require('log4js');


export default class Config {
    // todo: take config path
    readonly _data: any;

    constructor(filename: string) {
        // read off .env file and set process.env
        config();
        const fpath = path.join(__dirname, filename);
        try {
            const contents = fs.readFileSync(fpath, 'utf8');
            this._data = yaml.load(contents);    
        } catch (e) {
            throw Error(`Cannot parse YAML file at path ${fpath}, check if it exists`);
        }
        
        if (this._data.logging?.config) {
            log4js.configure(this._data.logging?.config)
        }
    } 

    get log_level(): string {
        return this.getProp("log_level") || this._data.logging?.level || 'OFF'; 
    }

    // TODO: make it type-safe
    public getProp(prop: string):string {
        // env variables take precedence
        return process.env[prop.toUpperCase()] || this._data.prop;
    }

    // TODO: make it type safe
    public get(): any {
        return this._data;
    }


}
