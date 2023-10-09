package net.brenkman.minecraft.mcmp.portal;

import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.brenkman.minecraft.mcmp.portal.frame.PortalFrameAllocator;
import net.brenkman.minecraft.mcmp.util.PortalHelper;
import net.minecraft.block.Block;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Direction;
import net.minecraft.world.World;

import java.util.Optional;

public class PortalPlacer {
    public static boolean attemptPortalIgnition(World world, BlockPos portalPos, IgnitionSource ignitionSource) {
        return attemptPortalIgnition(world, portalPos, PortalHelper.getClosestFrameBlock(world, portalPos), ignitionSource);
    }

    public static boolean attemptPortalIgnition(World world, BlockPos portalPos, BlockPos framePos, IgnitionSource ignitionSource) {
        Block foundationBlock = world.getBlockState(framePos).getBlock();
        PortalDefinition link = MultiversePortalRegistry.getPortalDefinitionFromBlock(foundationBlock);

        if (link == null || !link.doesIgnitionSourceMatch(ignitionSource))
            return false;
        return createPortal(link, foundationBlock, world, portalPos, framePos, ignitionSource);
    }

    private static boolean createPortal(PortalDefinition link, Block foundationBlock, World world, BlockPos portalPos, BlockPos framePos, IgnitionSource ignitionSource) {
        Optional<PortalFrameAllocator> optional = link.getFrameAllocatorFactory().createInstanceOfPortalFrameTester().getNewPortal(world, portalPos, Direction.Axis.X, foundationBlock);
        //is valid frame, and is correct size(if applicable)
        if (optional.isPresent()) {
//            if (link.getPortalPreIgniteEvent().attemptLight(ignitionSource.player, world, portalPos, framePos, ignitionSource)) {
                optional.get().lightPortal(foundationBlock);
//                link.getPortalIgniteEvent().afterLight(ignitionSource.player, world, portalPos, framePos, ignitionSource);
//            }
            return true;
        }
        return false;
    }
}
