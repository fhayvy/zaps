import { Router } from 'express';
import * as profileController from '../controllers/profile.controller';

const router = Router();

router.get('/profile', profileController.getMyProfile);
router.put('/profile', profileController.updateProfile);
router.get('/merchants', profileController.listMerchants);

export default router;
