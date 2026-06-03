import { Request, Response, NextFunction } from 'express';
import logger from '../utils/logger';

export class ApiError extends Error {
    constructor(public status: number, public message: string, public code?: string) {
        super(message);
    }
}

export const errorHandler = (err: any, req: Request, res: Response, next: NextFunction) => {
    const status = err.status || 500;
    const message = err.message || 'Internal Server Error';
    const code = err.code || 'INTERNAL_ERROR';

    if (status >= 500) {
        logger.error(`[ERROR] ${req.method} ${req.url}: ${err.message}`, {
            stack: err.stack,
            requestId: req.headers['x-request-id']
        });
    } else {
        logger.warn(`[WARN] ${req.method} ${req.url}: ${err.message}`, { status });
    }

    res.status(status).json({
        error: {
            message,
            status,
            code,
            requestId: req.headers['x-request-id'] || 'N/A'
        }
    });
};
