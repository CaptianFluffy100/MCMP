package net.brenkman.minecraft.mcmp.portal;

import net.brenkman.minecraft.mcmp.MCMPMod;
import net.brenkman.minecraft.mcmp.interfaces.EntityInMultiversePortal;
import net.brenkman.minecraft.mcmp.util.PortalHelper;
import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import net.minecraft.block.Block;
import net.minecraft.block.BlockState;
import net.minecraft.block.Blocks;
import net.minecraft.block.ShapeContext;
import net.minecraft.entity.Entity;
import net.minecraft.item.ItemStack;
import net.minecraft.particle.BlockStateParticleEffect;
import net.minecraft.sound.SoundCategory;
import net.minecraft.sound.SoundEvents;
import net.minecraft.state.StateManager;
import net.minecraft.state.property.EnumProperty;
import net.minecraft.state.property.Properties;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Direction;
import net.minecraft.util.math.random.Random;
import net.minecraft.util.shape.VoxelShape;
import net.minecraft.world.BlockView;
import net.minecraft.world.World;
import net.minecraft.world.WorldAccess;

public class MultiversePortalBlock extends Block {

    public static final EnumProperty<Direction.Axis> AXIS = Properties.AXIS;
    protected static final VoxelShape X_SHAPE = Block.createCuboidShape(0.0D, 0.0D, 6.0D, 16.0D, 16.0D, 10.0D);
    protected static final VoxelShape Z_SHAPE = Block.createCuboidShape(6.0D, 0.0D, 0.0D, 10.0D, 16.0D, 16.0D);
    protected static final VoxelShape Y_SHAPE = Block.createCuboidShape(0.0D, 6.0D, 0.0D, 16.0D, 10.0D, 16.0D);

    public MultiversePortalBlock(Settings settings) {
        super(settings);
        this.setDefaultState(this.getStateManager().getDefaultState().with(AXIS, Direction.Axis.X));
    }

    @Override
    public VoxelShape getOutlineShape(BlockState state, BlockView world, BlockPos pos, ShapeContext context) {
        return switch (state.get(AXIS)) {
            case Z -> Z_SHAPE;
            case Y -> Y_SHAPE;
            default -> X_SHAPE;
        };
    }

    public BlockState getStateForNeighborUpdate(BlockState state, Direction direction, BlockState newState, WorldAccess worldAccess, BlockPos pos, BlockPos posFrom) {
        // todo Needs helper function here
        return Blocks.AIR.getDefaultState();
    }

    protected void appendProperties(StateManager.Builder<Block, BlockState> builder) {
        builder.add(AXIS);
    }

    @Environment(EnvType.CLIENT)
    public ItemStack getPickStack(BlockView world, BlockPos pos, BlockState state) {
        return ItemStack.EMPTY;
    }

    @Override
    @Environment(EnvType.CLIENT)
    public void randomDisplayTick(BlockState state, World world, BlockPos pos, Random random) {
        if (random.nextInt(100) == 0) {
            world.playSound((double) pos.getX() + 0.5D, (double) pos.getY() + 0.5D, (double) pos.getZ() + 0.5D, SoundEvents.BLOCK_PORTAL_AMBIENT, SoundCategory.BLOCKS, 0.5F, random.nextFloat() * 0.4F + 0.8F, false);
        }

        for (int i = 0; i < 4; ++i) {
            double d = (double) pos.getX() + random.nextDouble();
            double e = (double) pos.getY() + random.nextDouble();
            double f = (double) pos.getZ() + random.nextDouble();
            double g = ((double) random.nextFloat() - 0.5) * 0.5;
            double h = ((double) random.nextFloat() - 0.5) * 0.5;
            double j = ((double) random.nextFloat() - 0.5) * 0.5;
            int k = random.nextInt(2) * 2 - 1;

            if (state.get(AXIS) == Direction.Axis.Y) {
                h = random.nextFloat() * 2.0f * (float) k;
            } else {
                if (world.getBlockState(pos.west()).isOf(this) || world.getBlockState(pos.east()).isOf(this)) {
                    f = (double) pos.getZ() + 0.5 + 0.25 * (double) k;
                    j = random.nextFloat() * 2.0f * (float) k;
                } else {
                    d = (double) pos.getX() + 0.5 + 0.25 * (double) k;
                    g = random.nextFloat() * 2.0f * (float) k;
                }
            }

            world.addParticle(new BlockStateParticleEffect(MCMPMod.MULTIVERSE_PORTAL_PARTICLE, PortalHelper.getPortalBase(world, pos).getDefaultState()), d, e, f, g, h, j);
        }
    }

    @Override
    public void onEntityCollision(BlockState state, World world, BlockPos pos, Entity entity) {
        EntityInMultiversePortal entityInPortal = (EntityInMultiversePortal) entity;
        entityInPortal.tickInPortal(pos.toImmutable());
        MCMPMod.LOGGER.info("I'm in the multiverse portal!!!!!!!!!!!!");
    }
}
