import winston from 'winston'
import expressWinston from 'express-winston'
import { Handler } from 'express'

// Creates basic winston logger.
function createDefaultLogger(): winston.Logger {
  const levels = {
    error: 0,
    warn: 1,
    info: 2,
    http: 3,
    debug: 4,
  }

  const level = () => {
    const env = process.env.NODE_ENV || 'development'
    const isDevelopment = env === 'development'
    return isDevelopment ? 'debug' : 'warn'
  }

  const colors = {
    error: 'red',
    warn: 'yellow',
    info: 'green',
    http: 'magenta',
    debug: 'white',
  }

  winston.addColors(colors)

  const format = winston.format.combine(
    winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss:ms' }),
    winston.format.colorize({ all: true }),
    winston.format.printf(
      (info) => `${info.timestamp} ${info.level}: ${info.message}`
    )
  )

  const transports = [new winston.transports.Console()]

  return winston.createLogger({
    level: level(),
    levels,
    format,
    transports,
  })
}

const Logger = createDefaultLogger()

export default Logger

// Creates Express-Winston logger handler.
export function httpLogger(): Handler {
  const opts: expressWinston.LoggerOptions = {
    transports: [new winston.transports.Console()],
    format: winston.format.combine(
      winston.format.colorize(),
      winston.format.json()
    ),
    meta: true,
    msg: 'HTTP {{req.method}} {{req.url}}',
    expressFormat: true,
    colorize: false,
  }

  return expressWinston.logger(opts)
}
