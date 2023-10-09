package net.brenkman.minecraft.mcmp;

import net.brenkman.minecraft.mcmp.api.MultiversePortalBuilder;
import net.brenkman.minecraft.mcmp.api.MultiversePortalRegistry;
import net.brenkman.minecraft.mcmp.config.Settings;
import net.brenkman.minecraft.mcmp.portal.IgnitionSource;
import net.brenkman.minecraft.mcmp.portal.MultiversePortalBlock;
import net.brenkman.minecraft.mcmp.portal.PortalPlacer;
import net.brenkman.minecraft.mcmp.portal.frame.VanillaFrameAllocator;
import net.brenkman.minecraft.mcmp.util.ColorUtil;
import net.fabricmc.api.ModInitializer;

import net.fabricmc.fabric.api.event.player.UseItemCallback;
import net.fabricmc.fabric.api.particle.v1.FabricParticleTypes;
import net.minecraft.block.AbstractBlock;
import net.minecraft.block.Blocks;
import net.minecraft.block.piston.PistonBehavior;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.item.Items;
import net.minecraft.particle.BlockStateParticleEffect;
import net.minecraft.particle.ParticleType;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.sound.BlockSoundGroup;
import net.minecraft.util.Identifier;
import net.minecraft.util.TypedActionResult;
import net.minecraft.util.hit.BlockHitResult;
import net.minecraft.util.hit.HitResult;
import net.minecraft.util.math.BlockPos;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class MCMPMod implements ModInitializer {
    // This logger is used to write text to the console and the log file.
    // It is considered best practice to use your mod id as the logger's name.
    // That way, it's clear which mod wrote info, warnings, and errors.
    public static final Logger LOGGER = LoggerFactory.getLogger("mcmp");
    public static final String MOD_ID = "mcmp";
    public static final MultiversePortalBlock MULTIVERSE_PORTAL_BLOCK = new MultiversePortalBlock(AbstractBlock.Settings.create().noCollision().ticksRandomly().strength(-1.0f).sounds(BlockSoundGroup.GLASS).luminance(state -> 11).pistonBehavior(PistonBehavior.BLOCK));

    static {
        Registry.register(Registries.BLOCK, new Identifier(MCMPMod.MOD_ID, "multiverse_portal_block"), MULTIVERSE_PORTAL_BLOCK);
    }

    // Should be with the client stuff, but apparently cannot be accessed from there
    public static final ParticleType<BlockStateParticleEffect> MULTIVERSE_PORTAL_PARTICLE = Registry.register(Registries.PARTICLE_TYPE, MCMPMod.MOD_ID + ":multiverse_portal_particle", FabricParticleTypes.complex(BlockStateParticleEffect.PARAMETERS_FACTORY));
    public static Identifier VANILLAPORTAL_FRAMEALLOCATOR = new Identifier(MOD_ID, "vanilla_frame_allocator");

    @Override
    public void onInitialize() {
        // This code runs as soon as Minecraft is in a mod-load-ready state.
        // However, some things (like resources) may still be uninitialized.
        // Proceed with mild caution.

        MultiversePortalRegistry.registerPortalFrameAllocatorFactory(VANILLAPORTAL_FRAMEALLOCATOR, VanillaFrameAllocator::new);
        UseItemCallback.EVENT.register(((player, world, hand) -> {
            ItemStack stack = player.getStackInHand(hand);
            if (!world.isClient) {
                Item item = stack.getItem();
                if (IgnitionSource.isRegisteredIgnitionSource(item)) {
                    HitResult hit = player.raycast(6, 1, false);
                    if (hit.getType() == HitResult.Type.BLOCK) {
                        BlockHitResult blockHit = (BlockHitResult) hit;
                        BlockPos pos = blockHit.getBlockPos();
                        if (PortalPlacer.attemptPortalIgnition(world, pos.offset(blockHit.getSide()), IgnitionSource.UseItemAsSource(item))) {
                            return TypedActionResult.success(stack);
                        };
                    }
                }
            }

            return TypedActionResult.pass(stack);
        }));

        MultiversePortalBuilder.beginPortal().frameBlock(Blocks.GOLD_BLOCK).tintColor(13000).lightWithItem(Items.STICK).tintColor(ColorUtil.getColorFromRGB(168, 50, 158)).registerPortal();

        LOGGER.info("Hello Fabric world!");

        Settings settings;
        try {
            settings = Settings.parseSettings();
            LOGGER.info("There are {} Portals", settings.portals.size());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        // Use Portals
        LOGGER.info("Portals: {}", settings.portals);
    }
}