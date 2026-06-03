import { Router } from 'express';
import * as fileController from '../controllers/file.controller';
const router = Router();

// Blueprint: In production, apply a multipart middleware (like multer) here
router.post('/upload', fileController.upload);
router.get('/:id', fileController.getFileMetadata);

export default router;
