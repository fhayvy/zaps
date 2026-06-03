import { Router } from 'express';
import * as auditController from '../controllers/audit.controller';

const router = Router();

router.get('/logs', auditController.getLogs);

export default router;
