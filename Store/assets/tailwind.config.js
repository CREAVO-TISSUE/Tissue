/** @type {import('tailwindcss').Config} */

export const mode = "jit";
export const module = {
  rules: [
    { test: /\.scss$/, use: ["style-loader", "css-loader", "sass-loader"] },
  ],
};
export const content = ["../templates/**/*.html"];
export const theme = {};
export const plugins = [];
