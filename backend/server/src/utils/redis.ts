import IORedis from 'ioredis';
import config from '../config';

const redisConfig = {
    host: config.redis.host,
    port: config.redis.port,
    password: config.redis.password,
    maxRetriesPerRequest: null,
};

export const connection = new IORedis(redisConfig);

connection.on('error', (err) => {
    console.error('Redis connection error:', err);
});

connection.on('connect', () => {
    console.log('Connected to Redis');
});

export default connection;
