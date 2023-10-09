package net.brenkman.minecraft.mcmp.api;

import net.brenkman.minecraft.mcmp.MCMPMod;
import net.brenkman.minecraft.mcmp.portal.IgnitionSource;
import net.brenkman.minecraft.mcmp.portal.PortalDefinition;
import net.brenkman.minecraft.mcmp.portal.frame.PortalFrameAllocator;
import net.minecraft.block.Block;
import net.minecraft.block.BlockState;
import net.minecraft.block.Blocks;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.util.Identifier;

import java.util.Collection;
import java.util.HashSet;
import java.util.concurrent.ConcurrentHashMap;

public class MultiversePortalRegistry {
    protected static final ConcurrentHashMap<Block, PortalDefinition> PORTAL_DEFINITIONS = new ConcurrentHashMap<>();
    private static final HashSet<Identifier> REGISTERED_IGNITION_SOURCE_ITEMS = new HashSet<>();
    protected static final ConcurrentHashMap<Identifier, PortalFrameAllocator.PortalFrameAllocatorFactory> portalFrameAllocatorFactories = new ConcurrentHashMap<>();

    public static PortalDefinition getPortalDefinitionFromBlock(Block block) {
        if (block == null) return null;
        return PORTAL_DEFINITIONS.getOrDefault(block, null);
    }

    public static boolean isBlockRegisteredFrame(BlockState blockState) {
        return PORTAL_DEFINITIONS.containsKey(blockState.getBlock());
    }

    public static boolean isBlockRegisteredFrame(Block block) {
        return PORTAL_DEFINITIONS.contains(block);
    }

    public static void registerPortalFrameAllocatorFactory(Identifier frameAllocatorID, PortalFrameAllocator.PortalFrameAllocatorFactory frameAllocatorFactory) {
        portalFrameAllocatorFactories.put(frameAllocatorID, frameAllocatorFactory);
    }

    public static PortalFrameAllocator.PortalFrameAllocatorFactory getPortalFrameAllocatorFactory(Identifier portalFrameAllocator) {
        return portalFrameAllocatorFactories.getOrDefault(portalFrameAllocator, null);
    }

    public static void registerItemAsIgnitionSource(Item item) {
        registerItemAsIgnitionSource(Registries.ITEM.getId(item));
    }

    public static boolean isItemRegisteredIgnitionSource(Item item) {
        return REGISTERED_IGNITION_SOURCE_ITEMS.contains(Registries.ITEM.getId(item));
    }
    public static void registerItemAsIgnitionSource(Identifier id) {
        REGISTERED_IGNITION_SOURCE_ITEMS.add(id);
    }

    public static void addPortalDefinition(Block frameBlock, PortalDefinition definition) {
        if (frameBlock == null) MCMPMod.LOGGER.error("Frame Block is null");
        if (definition.getPortalBlock() == null) MCMPMod.LOGGER.error("Portal block is null");

        if (PORTAL_DEFINITIONS.contains(frameBlock) || frameBlock.equals(Blocks.OBSIDIAN)) {
            MCMPMod.LOGGER.error("A portal is already registered with a frame made from: " + frameBlock);
        } else {
            PORTAL_DEFINITIONS.put(frameBlock, definition);
            if (definition.ignitionSource.ignitionSourceType == IgnitionSource.IgnitionSourceType.USEITEM) {
                registerItemAsIgnitionSource(definition.ignitionSource.ignitionSourceID);
            }
        }
    }

    public static Collection<PortalDefinition> getAllPortalDefitions() {
        return PORTAL_DEFINITIONS.values();
    }
}
