import { Response, NextFunction } from 'express';
import auditService from '../services/audit.service';
import logger from '../utils/logger';

export const auditLogging = (req: any, res: Response, next: NextFunction) => {
    // Only log state-changing requests
    if (['POST', 'PUT', 'PATCH', 'DELETE'].includes(req.method)) {
        const originalSend = res.send;

        res.send = function (body) {
            if (res.statusCode >= 200 && res.statusCode < 300) {
                // Log asynchronously after response is sent
                const actorId = req.user?.userId || 'anonymous';

                auditService.log({
                    actorId,
                    action: req.method,
                    resource: req.path,
                    metadata: {
                        query: req.query,
                        body: req.body,
                        status: res.statusCode,
                    },
                    ipAddress: req.ip || req.socket.remoteAddress,
                    userAgent: req.get('user-agent'),
                }).catch(err => logger.error('Audit logging failed:', { error: err.message, actorId }));
            }
            return originalSend.apply(res, arguments as any);
        };
    }
    next();
};
