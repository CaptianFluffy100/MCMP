package net.brenkman.minecraft.mcmp.config;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Settings {

    public final Resolver resolver;
    public final List<Portal> portals;

    public Settings(List<Portal> portals, Resolver resolver) {
        this.portals = portals;
        this.resolver = resolver;
    }

    public static Settings parseSettings() throws IOException {
        Gson g = new Gson();
        String json = readSettings();

        JsonObject jsonObj = new JsonParser().parse(json).getAsJsonObject();

        Settings settings = g.fromJson(jsonObj, Settings.class);

        return settings;
    }

    private static String readSettings() throws IOException {
        // Check if file exists
        // If not create and write to file
        if (!checkFile()) {
            writeToFile();
        }

        List<String> lines = Files.readAllLines(Paths.get(getSettingsPath()));
        String raw_data = String.join("\n", lines);

        return raw_data;
    }

    private static String getSettingsPath() {
        String dir = System.getProperty("user.dir")+"/config/mcmp.portals.json";
        return dir;
    }

    private static String getSettingsDir() {
        String dir = System.getProperty("user.dir")+"/config/";
        return dir;
    }

    private static boolean checkFile() {
        String path = getSettingsPath();
        File f = new File(path);
        return f.exists();
    }

    private static void writeToFile() throws IOException {
        File d = new File(getSettingsDir());
        if (!d.exists()) {
            Files.createDirectories(Paths.get(getSettingsDir()));
        }

        String path = getSettingsPath();
        File f = new File(path);
        FileWriter a = new FileWriter(f, true);
        BufferedWriter b = new BufferedWriter(a);
        b.write("{\"resolver\":{\"ip\":\"0.0.0.0\",\"port\":2000},\"portals\":[]}");
        b.close();
    }
}
