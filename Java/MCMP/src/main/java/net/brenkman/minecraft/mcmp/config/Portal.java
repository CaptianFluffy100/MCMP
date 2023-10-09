package net.brenkman.minecraft.mcmp.config;

public record Portal(String name, int color_r, int color_g, int color_b, String frameBlockId, String lightWithItemId, String command, Destination dest) {
}
