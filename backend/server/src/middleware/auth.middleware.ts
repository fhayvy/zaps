import { Request, Response, NextFunction } from 'express';
import authService from '../services/auth.service';

export const authenticate = (req: any, res: Response, next: NextFunction) => {
    const authHeader = req.headers.authorization;
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
        return res.status(401).json({ error: 'Unauthorized: missing or invalid token' });
    }

    const token = authHeader.split(' ')[1];
    try {
        const decoded = authService.verifyToken(token);
        req.user = decoded;
        next();
    } catch (err) {
        return res.status(401).json({ error: 'Unauthorized: token expired or invalid' });
    }
};
