package net.brenkman.minecraft.mcmp.network;

import net.brenkman.minecraft.mcmp.MCMPMod;
import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking;
import net.minecraft.util.Identifier;

public class NetworkManager {
    @Environment(EnvType.CLIENT)
    public static void registerClientReceivePortalDefinition() {
        ClientPlayNetworking.registerGlobalReceiver(ServerNetworkManager.SYNC_PORTAL_PACKET_ID, ((client, handler, buf, responseSender) -> {
            Identifier frameBlock = buf.readIdentifier();
            int color = buf.readInt();
            MCMPMod.LOGGER.info("Got a portal sync definition: " + frameBlock);
        }));
    }
}
