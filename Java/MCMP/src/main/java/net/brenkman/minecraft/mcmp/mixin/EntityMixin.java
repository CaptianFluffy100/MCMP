package net.brenkman.minecraft.mcmp.mixin;

import net.brenkman.minecraft.mcmp.interfaces.EntityInMultiversePortal;
import net.minecraft.entity.Entity;
import net.minecraft.nbt.NbtCompound;
import net.minecraft.util.math.BlockPos;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Unique;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

@Mixin(Entity.class)
public abstract class EntityMixin implements EntityInMultiversePortal {
    @Unique
    int timeInPortal = 0, maxTimeInPortal = 80, cooldown = 0;

    @Unique
    private BlockPos inPortalPos;

    @Unique
    @Override
    public void tickInPortal(BlockPos portalPos) {
        cooldown = 10;
        inPortalPos = portalPos;
    }

    @Unique
    @Override
    public BlockPos getInPortalPos() {
        return inPortalPos;
    }

    @Inject(method = "tick", at = @At(value = "TAIL"))
    public void MCMPtick(CallbackInfo ci) {
        if (cooldown > 0) {
            cooldown--;
            timeInPortal = Math.min(timeInPortal + 1, maxTimeInPortal);
            if (cooldown <= 0) {
            }
        }
    }

    @Inject(method = "readNbt", at = @At(value = "TAIL"))
    public void CPAreadCustomPortalFromTag(NbtCompound tag, CallbackInfo ci) {
        this.cooldown = tag.getInt("mcmpCooldown");
    }

    @Inject(method = "writeNbt", at = @At(value = "RETURN"))
    public void CPAwriteCustomPortalToTag(NbtCompound tag, CallbackInfoReturnable<NbtCompound> cir) {
        cir.getReturnValue().putInt("mcmpCooldown", cooldown);
    }
}
