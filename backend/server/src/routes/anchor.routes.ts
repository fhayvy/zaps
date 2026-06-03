import { Router } from 'express';
import * as anchorController from '../controllers/anchor.controller';

const router = Router();

router.post('/withdraw', anchorController.createWithdrawal);
router.get('/withdraw/:id', anchorController.getStatus);

export default router;
