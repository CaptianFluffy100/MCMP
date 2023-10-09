package net.brenkman.minecraft.mcmp.network;

import io.netty.buffer.Unpooled;
import net.brenkman.minecraft.mcmp.MCMPMod;
import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.brenkman.minecraft.mcmp.portal.PortalDefinition;
import net.fabricmc.api.DedicatedServerModInitializer;
import net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents;
import net.minecraft.network.PacketByteBuf;
import net.minecraft.network.packet.Packet;
import net.minecraft.network.packet.s2c.play.CustomPayloadS2CPacket;
import net.minecraft.util.Identifier;

public class ServerNetworkManager implements DedicatedServerModInitializer {
    public static final Identifier SYNC_PORTAL_PACKET_ID = new Identifier(MCMPMod.MOD_ID, "sync_portal_packet_id");
    public static final Identifier PLACE_PACKET_ID = new Identifier(MCMPMod.MOD_ID, "place_portal_packet_id");

    private static Packet<?> createPortalDefinitionPacket(PortalDefinition definition) {
        PacketByteBuf buf = new PacketByteBuf(Unpooled.buffer());
        buf.writeIdentifier(definition.blockID);
        buf.writeInt(definition.color);
        return new CustomPayloadS2CPacket(SYNC_PORTAL_PACKET_ID, buf);
    }

    @Override
    public void onInitializeServer() {
        MCMPMod.LOGGER.info("Initializing server logic");
        ServerPlayConnectionEvents.JOIN.register(((handler, sender, server) -> {
            MCMPMod.LOGGER.info("Syncing portal definitions to newly joined player: " + handler.getPlayer().getName());
            for (PortalDefinition definition : MultiversePortalRegistry.getAllPortalDefitions()) {
                MCMPMod.LOGGER.info("Sending packet about portal definition: " + definition.blockID);
                sender.sendPacket(createPortalDefinitionPacket(definition));
            }
        }));
    }
}
