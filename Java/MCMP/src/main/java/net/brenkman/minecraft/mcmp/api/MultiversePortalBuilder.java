package net.brenkman.minecraft.mcmp.api;

import net.brenkman.minecraft.mcmp.portal.MultiversePortalBlock;
import net.brenkman.minecraft.mcmp.portal.IgnitionSource;
import net.brenkman.minecraft.mcmp.portal.PortalDefinition;
import net.minecraft.block.Block;
import net.minecraft.fluid.Fluid;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.util.Identifier;

public class MultiversePortalBuilder {
    private final PortalDefinition portalDefinition;
    private MultiversePortalBuilder(PortalDefinition portalDefinition) {
        this.portalDefinition = portalDefinition;
    }

    public static MultiversePortalBuilder beginPortal() {
        return beginPortal(new PortalDefinition());
    }

    public static MultiversePortalBuilder beginPortal(PortalDefinition definition) {
        return new MultiversePortalBuilder(definition);
    }

    /**
     * Register the portal when completed.
     * This should be called last, only when you are finished configuring the portal
     *
     * @return the raw PortalDefinition created from this builder.
     */
    public PortalDefinition registerPortal() {
        MultiversePortalRegistry.addPortalDefinition(Registries.BLOCK.get(portalDefinition.blockID), portalDefinition);
        return portalDefinition;
    }

    /**
     * Specify the Block Identifier to be used as the Frame
     *
     * @param blockID Block identifier of the portal's frame block
     */
    public MultiversePortalBuilder frameBlock(Identifier blockID) {
        portalDefinition.blockID = blockID;
        return this;
    }

    /**
     * Specify the Block to be used as the Frame
     *
     * @param block The Block to be used as the portal's frame block
     */
    public MultiversePortalBuilder frameBlock(Block block) {
        portalDefinition.blockID = Registries.BLOCK.getId(block);
        return this;
    }

    /**
     * Specify the color to be used to tint the portal block.
     *
     * @param color Single Color int value used for tinting. See {@link net.minecraft.util.DyeColor}
     */
    public MultiversePortalBuilder tintColor(int color) {
        portalDefinition.color = color;
        return this;
    }

    /**
     * Specify the color in RGB to be used to tint the portal block.
     */
//    public MultiversePortalBuilder tintColor(int r, int g, int b) {
//        portalDefinition.color = ColorUtil.getColorFromRGB(r, g, b);
//        return this;
//    }

    /**
     * This portal will be ignited by water
     */
    public MultiversePortalBuilder lightWithWater() {
        portalDefinition.ignitionSource = IgnitionSource.WATER;
        return this;
    }

    /**
     * This portal will be ignited by an item
     *
     * @param item Item to be used to ignite the portal
     */
    public MultiversePortalBuilder lightWithItem(Item item) {
        portalDefinition.ignitionSource = IgnitionSource.UseItemAsSource(item);
        return this;
    }

    /**
     * This portal will be ignited by a fluid.
     *
     * @param fluid Fluid to be used to ignite the portal
     */
    public MultiversePortalBuilder lightWithFluid(Fluid fluid) {
        portalDefinition.ignitionSource = IgnitionSource.FluidAsSource(fluid);
        return this;
    }

    /**
     * Specify a Custom Ignition Source to be used to ignite the portal. You must manually trigger the ignition yourself.
     */
    public MultiversePortalBuilder customIgnitionSource(Identifier customSourceID) {
        portalDefinition.ignitionSource = IgnitionSource.CustomSource(customSourceID);
        return this;
    }

    /**
     * Specify a Custom Ignition Source to be used to ignite the portal. You must manually trigger the ignition yourself.
     */
    public MultiversePortalBuilder customIgnitionSource(IgnitionSource ignitionSource) {
        portalDefinition.ignitionSource = ignitionSource;
        return this;
    }

//    /**
//     * Specify the forced size of the portal.
//     * Portal will only be ignitable for these exact dimensions
//     *
//     * @param width  Forced width of portal
//     * @param height Forced height of portal
//     */
//    public MultiversePortalBuilder forcedSize(int width, int height) {
//        portalDefinition.forcedWidth = width;
//        portalDefinition.forcedHeight = height;
//        return this;
//    }

    /**
     * Specify a custom block to be used as the portal block. Block must extend CustomPortalBlock.
     */
    public MultiversePortalBuilder customPortalBlock(MultiversePortalBlock portalBlock) {
        portalDefinition.setPortalBlock(portalBlock);
        return this;
    }

    /**
     * Specify the dimension this portal will return you to.
     *
     * @param returnDimID              Identifer of the dimmension the portal will return you to when leaving destination
     * @param onlyIgnitableInReturnDim Should this portal only be ignitable in returnDimID
     */
//    public MultiversePortalBuilder returnDim(Identifier returnDimID, boolean onlyIgnitableInReturnDim) {
//        portalDefinition.returnDimID = returnDimID;
//        portalDefinition.onlyIgnitableInReturnDim = onlyIgnitableInReturnDim;
//        return this;
//    }

    /**
     * Specify that this portal can only be ignited in the Overworld.
     * Attempting to light it in other dimensions will fail.
     */
//    public MultiversePortalBuilder onlyLightInOverworld() {
//        portalDefinition.onlyIgnitableInReturnDim = true;
//        return this;
//    }

    /**
     * Specify that this is a flat portal (end portal style).
     */
//    public MultiversePortalBuilder flatPortal() {
//        portalDefinition.portalFrameTester = CustomPortalsMod.FLATPORTAL_FRAMETESTER;
//        return this;
//    }

    /**
     * Specify a custom portal frame tester to be used.
     */
    public MultiversePortalBuilder customFrameTester(Identifier frameTester) {
        portalDefinition.portalFrameAllocator = frameTester;
        return this;
    }
}
