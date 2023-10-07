package net.brenkman.minecraft.mcmp.util;

import net.brenkman.minecraft.mcmp.portal.MultiversePortalBlock;
import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.minecraft.block.*;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Direction;
import net.minecraft.world.World;

public class PortalHelper {
    public static boolean isInstanceMultiversePortal(World world, BlockPos pos) {
        return isInstanceMultiversePortal(world.getBlockState(pos));
    }

    public static boolean isInstanceMultiversePortal(BlockState blockState) {
        return blockState.getBlock() instanceof MultiversePortalBlock;
    }

    public static boolean isInstanceOfPortalFrame(World world, BlockPos pos) {
        if (world.isInBuildLimit(pos))
            return MultiversePortalRegistry.isBlockRegisteredFrame(world.getBlockState(pos));
        return false;
    }

    public static Direction.Axis getAxisFrom(BlockState state) {
        if (isInstanceMultiversePortal(state))
            return state.get(MultiversePortalBlock.AXIS);
        if (state.getBlock() instanceof NetherPortalBlock)
            return state.get(NetherPortalBlock.AXIS);
        if (state.getBlock() instanceof EndPortalBlock)
            return Direction.Axis.Y;
        return Direction.Axis.X;
    }

    public static BlockState blockWithAxis(BlockState state, Direction.Axis axis) {
        if (state.getBlock() instanceof MultiversePortalBlock)
            return state.with(MultiversePortalBlock.AXIS, axis);
        return state;
    }

    public static Block getPortalBase(World world, BlockPos pos) {
        if (isInstanceMultiversePortal(world, pos)) {
            Direction.Axis axis = getAxisFrom(world.getBlockState(pos));

            if (axis != Direction.Axis.Y) {
                if (isInstanceOfPortalFrame(world, pos.down()))
                    return world.getBlockState(pos.down()).getBlock();
                if (isInstanceOfPortalFrame(world, pos.up()))
                    return world.getBlockState(pos.up()).getBlock();
            } else axis = Direction.Axis.Z;

            if (isInstanceOfPortalFrame(world, pos.offset(axis, -1)))
                return world.getBlockState(pos.offset(axis, -1)).getBlock();
            if (isInstanceOfPortalFrame(world, pos.offset(axis, 1)))
                return world.getBlockState(pos.offset(axis, 1)).getBlock();

            return getPortalBase(world, pos.offset(axis, -1));
        } else if (isInstanceOfPortalFrame(world, pos))
            return world.getBlockState(pos).getBlock();
//        if (isInstanceMultiversePortal(world, pos)) {
//            return ((MultiversePortalBlock) world.getBlockState(pos).getBlock()).getPortalBase(world, pos);
//        } else if (isInstanceOfPortalFrame(world, pos))
//            return world.getBlockState(pos).getBlock();

        return Blocks.AIR;
    }

    public static BlockPos getClosestFrameBlock(World world, BlockPos pos) {
        if (isInstanceOfPortalFrame(world, pos.down()))
            return pos.down();
        if (isInstanceOfPortalFrame(world, pos.east()))
            return pos.east();
        if (isInstanceOfPortalFrame(world, pos.west()))
            return pos.west();
        if (isInstanceOfPortalFrame(world, pos.north()))
            return pos.north();
        if (isInstanceOfPortalFrame(world, pos.south()))
            return pos.south();
        if (isInstanceOfPortalFrame(world, pos.up()))
            return pos.up();
        return pos;
    }
}
