package net.brenkman.minecraft.mcmp;

import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.brenkman.minecraft.mcmp.network.NetworkManager;
import net.brenkman.minecraft.mcmp.portal.PortalDefinition;
import net.brenkman.minecraft.mcmp.util.PortalHelper;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import net.fabricmc.fabric.api.blockrenderlayer.v1.BlockRenderLayerMap;
import net.fabricmc.fabric.api.client.particle.v1.ParticleFactoryRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.ColorProviderRegistry;
import net.minecraft.block.Block;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.render.RenderLayer;

@Environment(EnvType.CLIENT)
public class MCMPClient implements ClientModInitializer {

    @Override
    public void onInitializeClient() {
        MCMPMod.LOGGER.info("Initializing client...");
        BlockRenderLayerMap.INSTANCE.putBlock(MCMPMod.MULTIVERSE_PORTAL_BLOCK, RenderLayer.getTranslucent());
        ColorProviderRegistry.BLOCK.register((state, world, pos, tintIndex) -> {
            if (pos != null) {
                Block block = PortalHelper.getPortalBase(MinecraftClient.getInstance().world, pos.toImmutable());
                PortalDefinition definition = MultiversePortalRegistry.getPortalDefinitionFromBlock(block);
                if (definition != null) return definition.color;
            }
            return 1908001;
        }, MCMPMod.MULTIVERSE_PORTAL_BLOCK);

        ParticleFactoryRegistry.getInstance().register(MCMPMod.MULTIVERSE_PORTAL_PARTICLE, MultiversePortalParticle.Factory::new);
        NetworkManager.registerClientReceivePortalDefinition();
    }
}