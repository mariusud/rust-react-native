import { StatusBar } from "expo-status-bar";
import { StyleSheet, Text, View } from "react-native";
import { hello, rustAdd } from "./modules/rust-module";
import { useEffect, useState } from "react";

export default function App() {
  const [value, setValue] = useState<null | number>(null);
  useEffect(() => {
    console.log("Calling rustAdd...");
    async function doFetch() {
      console.log("Fetching value from Rust...");
      const result = await rustAdd(40, 12);
      console.log("Received value from Rust:", result);
      setValue(result);
    }
    doFetch();
  }, []);
  return (
    <View style={styles.container}>
      <Text style={styles.text}>{hello()}</Text>
      <Text style={styles.text}>
        {value === null ? "Loading..." : `The value is: ${value}`}
      </Text>
      <StatusBar style="auto" />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#FFFFFF",
    alignItems: "center",
    justifyContent: "center",
  },
  text: {
    fontSize: 42,
  },
});
