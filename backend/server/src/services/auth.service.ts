import jwt from 'jsonwebtoken';
import bcrypt from 'bcryptjs';
import config from '../config';
import prisma from '../utils/prisma';

class AuthService {
    private readonly jwtSecret = config.jwtSecret;

    async login(userId: string, pin: string) {
        const user = await prisma.user.findUnique({
            where: { userId },
        });

        if (!user) {
            throw new Error('User not found');
        }

        const isValid = await bcrypt.compare(pin, user.pinHash);
        if (!isValid) {
            throw new Error('Invalid PIN');
        }

        const accessToken = this.generateAccessToken(user.userId, user.role);
        const refreshToken = this.generateRefreshToken(user.userId);

        return {
            accessToken,
            refreshToken,
            user: {
                userId: user.userId,
                stellarAddress: user.stellarAddress,
                role: user.role,
            },
        };
    }

    generateAccessToken(userId: string, role: string) {
        return jwt.sign({ userId, role }, this.jwtSecret, { expiresIn: '1h' });
    }

    generateRefreshToken(userId: string) {
        return jwt.sign({ userId }, this.jwtSecret, { expiresIn: '7d' });
    }

    verifyToken(token: string) {
        return jwt.verify(token, this.jwtSecret);
    }
}

export default new AuthService();
