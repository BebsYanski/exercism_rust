export function decodedValue(colorNames:string[]) {
return Number(`${COLORS.indexOf(colorNames[0])}${COLORS.indexOf(colorNames[1])}`);
}
const COLORS:string[] = ["black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"];