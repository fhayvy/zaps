import prisma from '../utils/prisma';

class ProfileService {
    async getProfile(userId: string) {
        return prisma.userProfile.findUnique({
            where: { userId },
        });
    }

    async updateProfile(userId: string, data: { displayName?: string; bio?: string; country?: string; avatarUrl?: string }) {
        return prisma.userProfile.update({
            where: { userId },
            data,
        });
    }

    async listMerchants() {
        return prisma.merchant.findMany({
            where: { active: true },
        });
    }
}

export default new ProfileService();
