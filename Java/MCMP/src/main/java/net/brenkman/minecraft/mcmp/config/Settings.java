package net.brenkman.minecraft.mcmp.config;

import com.google.gson.Gson;
import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import com.google.gson.reflect.TypeToken;
import org.apache.commons.io.IOUtils;
import net.fabricmc.mapping.reader.v2.MappingGetter;

import java.io.*;
import java.lang.reflect.Type;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import org.slf4j.Logger;

public class Settings {

    public static List<Portal> parseJsonString() throws IOException {
        Type listType = new TypeToken<List<Portal>>() {}.getType();
        Gson g = new Gson();
        String json = readSettings();

        JsonObject jsonObj = new JsonParser().parse(json).getAsJsonObject();
        JsonArray jsonArray = jsonObj.getAsJsonArray("portals");

        List<Portal> portals = g.fromJson(jsonArray, listType);

        return portals;
    }

    public static String readSettings() throws IOException {
        // Check if file exists
        // If not create and write to file
        if (!checkFile()) {
            writeToFile();
        }

        List<String> lines = Files.readAllLines(Paths.get(getSettingsPath()));
        String raw_data = String.join("\n", lines);

        return raw_data;
    }

    public static String getSettingsPath() {
        String dir = System.getProperty("user.dir")+"/Config/mcmp.portals.json";
        return dir;
    }

    public static boolean checkFile() {
        String path = getSettingsPath();
        File f = new File(path);
        return f.exists();
    }

    public static void writeToFile() throws IOException {
        String path = getSettingsPath();
        File f = new File(path);
        // f.getParentFile().mkdirs();
        FileWriter a = new FileWriter(f, true);
        BufferedWriter b = new BufferedWriter(a);
        b.write("{\"portals\":[]}");
        b.close();
    }
}
