import { Request, Response, NextFunction } from 'express';
import profileService from '../services/profile.service';
import { ApiError } from '../middleware/error.middleware';

export const getMyProfile = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const userId = (req as any).user.userId;
        const profile = await profileService.getProfile(userId);
        if (!profile) throw new ApiError(404, 'Profile not found');
        res.status(200).json(profile);
    } catch (error) {
        next(error);
    }
};

export const updateProfile = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const userId = (req as any).user.userId;
        const profile = await profileService.updateProfile(userId, req.body);
        res.status(200).json(profile);
    } catch (error) {
        next(error);
    }
};

export const listMerchants = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const merchants = await profileService.listMerchants();
        res.status(200).json(merchants);
    } catch (error) {
        next(error);
    }
};
