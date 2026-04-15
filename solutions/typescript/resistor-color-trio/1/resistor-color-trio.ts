export function decodedResistorValue(colors:string[]) {
  // throw new Error('Remove this line and implement the function')
  let result:number = Number(`${COLORS.indexOf(colors[0])}${COLORS.indexOf(colors[1])}`) * (10**COLORS.indexOf(colors[2]));

  if (result >= 1000000000){
    result /= 1000000000;
    return `${result} gigaohms`;
  }else if (result >= 1000000){
    result /= 1000000;
    return `${result} megaohms`;
  }else if (result >= 1000){
    result /= 1000;
    return `${result} kiloohms`;
  }else{
return `${result} ohms`;    
  }

}

const COLORS:string[] = ['black','brown','red','orange','yellow','green','blue','violet','grey','white'];