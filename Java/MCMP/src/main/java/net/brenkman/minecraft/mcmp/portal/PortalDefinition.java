package net.brenkman.minecraft.mcmp.portal;

import net.brenkman.minecraft.mcmp.MCMPMod;
import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.brenkman.minecraft.mcmp.event.IgnitionEvent;
import net.brenkman.minecraft.mcmp.portal.frame.PortalFrameAllocator;
import net.minecraft.block.Block;
import net.minecraft.util.Identifier;

public class PortalDefinition {
    public Identifier blockID;
    public IgnitionSource ignitionSource = IgnitionSource.FIRE;
    private MultiversePortalBlock block = MCMPMod.MULTIVERSE_PORTAL_BLOCK;
    public int color;
    public int forceWidth, forceHeight;
    public Identifier portalFrameAllocator = MCMPMod.VANILLAPORTAL_FRAMEALLOCATOR;

    private IgnitionEvent portalIgniteEvent = (player, world, portalPos, framePos, portalIgnitionSource) -> {
    };
//    private PortalPreIgniteEvent portalPreIgniteEvent = (player, world, portalPos, framePos, portalIgnitionSource) -> true;

    public PortalDefinition() {}

    public PortalDefinition(Identifier blockID, int color) {
        this.blockID = blockID;
        this.color = color;
    }

    public Block getPortalBlock() {
        return this.block;
    }

    public void setPortalBlock(MultiversePortalBlock block) {
        this.block = block;
    }

    // TODO implement the equals function so we don't have this here
    public boolean doesIgnitionSourceMatch(IgnitionSource ignitionAttemptSource) {
        return this.ignitionSource.ignitionSourceType == ignitionAttemptSource.ignitionSourceType && ignitionSource.ignitionSourceID == ignitionAttemptSource.ignitionSourceID;
    }

    public PortalFrameAllocator.PortalFrameAllocatorFactory getFrameAllocatorFactory() {
        return MultiversePortalRegistry.getPortalFrameAllocatorFactory(portalFrameAllocator);
    }
}
