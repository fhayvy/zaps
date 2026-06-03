import { Request, Response, NextFunction } from 'express';
import notificationService from '../services/notification.service';
import prisma from '../utils/prisma';
import { ApiError } from '../middleware/error.middleware';

export const getMyNotifications = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const userId = (req as any).user.userId;
        const notifications = await notificationService.getNotifications(userId);
        res.status(200).json(notifications);
    } catch (error) {
        next(error);
    }
};

export const markAsRead = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const { id } = req.params;
        const userId = (req as any).user.userId;

        const notification = await prisma.notification.update({
            where: { id, userId },
            data: { read: true }
        });

        res.status(200).json(notification);
    } catch (error) {
        next(error);
    }
};
