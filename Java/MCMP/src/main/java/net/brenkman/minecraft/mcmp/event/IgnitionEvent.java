package net.brenkman.minecraft.mcmp.event;

import net.brenkman.minecraft.mcmp.portal.IgnitionSource;
import net.minecraft.entity.player.PlayerEntity;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

@FunctionalInterface
public interface IgnitionEvent {
    void onIgnite(PlayerEntity player, World world, BlockPos pos, BlockPos framePos, IgnitionSource ignitionSource);
}
