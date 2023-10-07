package net.brenkman.minecraft.mcmp;

import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.brenkman.minecraft.mcmp.portal.PortalDefinition;
import net.brenkman.minecraft.mcmp.util.ColorUtil;
import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import net.minecraft.block.Block;
import net.minecraft.client.particle.Particle;
import net.minecraft.client.particle.ParticleFactory;
import net.minecraft.client.particle.PortalParticle;
import net.minecraft.client.particle.SpriteProvider;
import net.minecraft.client.world.ClientWorld;
import net.minecraft.particle.BlockStateParticleEffect;
import org.jetbrains.annotations.Nullable;

public class MultiversePortalParticle extends PortalParticle {

    protected MultiversePortalParticle(ClientWorld clientWorld, double d, double e, double f, double g, double h, double i) {
        super(clientWorld, d, e, f, g, h, i);
    }

    @Environment(EnvType.CLIENT)
    public static class Factory implements ParticleFactory<BlockStateParticleEffect> {
        private final SpriteProvider spriteProvider;

        public Factory(SpriteProvider spriteProvider) {
            this.spriteProvider = spriteProvider;
        }

        @Nullable
        @Override
        public Particle createParticle(BlockStateParticleEffect blockStateParticleEffect, ClientWorld world, double x, double y, double z, double velocityX, double velocityY, double velocityZ) {
            MultiversePortalParticle portalParticle = new MultiversePortalParticle(world, x, y, z, velocityX, velocityY, velocityZ);
            portalParticle.setSprite(this.spriteProvider);
            Block block = blockStateParticleEffect.getBlockState().getBlock();
            PortalDefinition definition = MultiversePortalRegistry.getPortalDefinitionFromBlock(block);

            if (definition != null) {
                float[] rgb = ColorUtil.getColorForBlock(definition.color);
                portalParticle.setColor(rgb[0], rgb[1], rgb[2]);
            }

            return portalParticle;
        }
    }
}
