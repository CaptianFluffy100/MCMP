package net.brenkman.minecraft.mcmp;

import net.brenkman.minecraft.mcmp.config.Settings;
import net.fabricmc.api.ModInitializer;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class MCMP implements ModInitializer {
    // This logger is used to write text to the console and the log file.
    // It is considered best practice to use your mod id as the logger's name.
    // That way, it's clear which mod wrote info, warnings, and errors.
    public static final Logger LOGGER = LoggerFactory.getLogger("MCMP");

    @Override
    public void onInitialize() {
        // This code runs as soon as Minecraft is in a mod-load-ready state.
        // However, some things (like resources) may still be uninitialized.
        // Proceed with mild caution.
        Settings settings;
        try {
            settings = Settings.parseSettings();
            LOGGER.info("There are {} Portals", settings.portals.size());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        // Use GLaDOS Settings (resolver)
        LOGGER.info("GLaDOS: {}", settings.resolver);
        // Use Portals
        LOGGER.info("Portals: {}", settings.portals);
    }
}