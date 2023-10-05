package net.brenkman.minecraft.mcmp.commands;

import net.minecraft.entity.Entity;

public class portalCommandParser {
    public static String parseCommand(String command, Entity entity) {
        command = command.replace("%PLAYERNAME%", entity.getName().getString());
        command = command.replace("%PLAYERUUID%", entity.getUuid().toString());
        command = command.replace("%PLAYERPOS%", entity.getPos().toString());
        command = command.replace("%PLAYERSERVER%", "Server IP: " + entity.getServer().getServerIp().toString() + ", Server MOTD: " + entity.getServer().getServerMotd() + ", Server Version: " + entity.getServer().getVersion() + ", Player Count: " + entity.getServer().getCurrentPlayerCount() + "/" + entity.getServer().getMaxPlayerCount());
        return command;
    }
}
