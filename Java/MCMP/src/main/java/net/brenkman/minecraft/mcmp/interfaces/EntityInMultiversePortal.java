package net.brenkman.minecraft.mcmp.interfaces;

import net.minecraft.util.math.BlockPos;

public interface EntityInMultiversePortal {
    int getTimeInPortal();
    void tickInPortal(BlockPos pos);
    BlockPos getInPortalPos();
}
