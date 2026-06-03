import { Request, Response, NextFunction } from 'express';
import connection from '../utils/redis';
import logger from '../utils/logger';
import { ApiError } from './error.middleware';

/**
 * Basic fixed-window rate limiting using Redis.
 * Ported logic from rate_limit_service.rs
 */
export const rateLimit = (limit: number, windowSeconds: number = 60) => {
    return async (req: Request, res: Response, next: NextFunction) => {
        const ip = req.ip || req.socket.remoteAddress || 'unknown';
        const key = `rate_limit:${ip}`;

        try {
            const current = await connection.incr(key);

            if (current === 1) {
                await connection.expire(key, windowSeconds);
            }

            if (current > limit) {
                logger.warn(`Rate limit exceeded for IP: ${ip}`, { current, limit });
                return next(new ApiError(429, 'Too many requests', 'RATE_LIMIT_EXCEEDED'));
            }

            next();
        } catch (err: any) {
            logger.error('Rate limiting internal error:', { error: err.message });
            // Fail open to avoid blocking users on Redis failure
            next();
        }
    };
};
