package net.brenkman.minecraft.mcmp.portal;

import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.minecraft.block.Blocks;
import net.minecraft.fluid.Fluid;
import net.minecraft.fluid.Fluids;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.registry.tag.FluidTags;
import net.minecraft.util.Identifier;

import java.util.HashSet;

public class IgnitionSource {
    public final static IgnitionSource FIRE = new IgnitionSource(IgnitionSourceType.PLACEBLOCK, Registries.BLOCK.getId(Blocks.FIRE));
    public final static IgnitionSource WATER = FluidAsSource(Fluids.WATER);
    public enum IgnitionSourceType {
        USEITEM, PLACEBLOCK, FLUID, CUSTOM
    }
    public IgnitionSourceType ignitionSourceType;
    public Identifier ignitionSourceID;

    public IgnitionSource(IgnitionSourceType ignitionSourceType, Identifier id) {
        this.ignitionSourceType = ignitionSourceType;
        this.ignitionSourceID = id;
    }

    public static IgnitionSource UseItemAsSource(Item item) {
        return new IgnitionSource(IgnitionSourceType.USEITEM, Registries.ITEM.getId(item));
    }

    public static IgnitionSource FluidAsSource(Fluid fluid) {
        return new IgnitionSource(IgnitionSourceType.FLUID, Registries.FLUID.getId(fluid));
    }

    public static IgnitionSource CustomSource(Identifier ignitionSourceID) {
        return new IgnitionSource(IgnitionSourceType.CUSTOM, ignitionSourceID);
    }

    public boolean isWater() {
        return Registries.FLUID.get(ignitionSourceID).isIn(FluidTags.WATER);
    }

    public boolean isLava() {
        return Registries.FLUID.get(ignitionSourceID).isIn(FluidTags.LAVA);
    }

    public static boolean isRegisteredIgnitionSource(Item item) {
        return MultiversePortalRegistry.isItemRegisteredIgnitionSource(item);
    }

//    public IgnitionSource withPlayer(PlayerEntity player) {
//        this.player = player;
//        return this;
//    }

    // TODO: Implement a condition with an ignition source, like only a certian player, or based on location or time etc.
    // must be met in order for the ignition to be successful.
}
