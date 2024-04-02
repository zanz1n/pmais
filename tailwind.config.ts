import { skeleton } from "@skeletonlabs/tw-plugin";
import { join } from "path";
import { Config } from "tailwindcss";

const config = {
  darkMode: "class",

  content: [
    "./src/**/*.{html,js,svelte,ts}",
    join(
      require.resolve("@skeletonlabs/skeleton"),
      "../**/*.{html,js,svelte,ts}"
    )
  ],

  theme: {
    extend: {},
  },

  plugins: [
    skeleton({
      themes: {
        preset: [
          { name: "skeleton", enhancements: true },
          { name: "wintry", enhancements: true },
          { name: "vintage", enhancements: true },
          { name: "crimson", enhancements: true },
        ]
      }
    })
  ],
} satisfies Config;

export default config;
