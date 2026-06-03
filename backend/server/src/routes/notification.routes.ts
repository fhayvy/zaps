import { Router } from 'express';
import * as notificationController from '../controllers/notification.controller';

const router = Router();

router.get('/', notificationController.getMyNotifications);
router.patch('/:id/read', notificationController.markAsRead);

export default router;
