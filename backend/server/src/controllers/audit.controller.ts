import { Request, Response, NextFunction } from 'express';
import auditService from '../services/audit.service';
import { ApiError } from '../middleware/error.middleware';

export const getLogs = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const { actorId, limit } = req.query;
        const parsedLimit = limit ? parseInt(limit as string, 10) : 50;

        // Restriction: Only admins can view all logs, users can view their own
        const currentUser = (req as any).user;
        let targetActorId = actorId as string | undefined;

        if (currentUser.role !== 'ADMIN') {
            targetActorId = currentUser.userId;
        }

        const logs = await auditService.getLogs(targetActorId, parsedLimit);
        res.status(200).json(logs);
    } catch (error) {
        next(error);
    }
};
