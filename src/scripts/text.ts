export const getContentHeight = (target: HTMLElement): number => {
  const style = getComputedStyle(target as Element, null);
  const box_sizing = style.getPropertyValue("box-sizing");
  let height = parseInt(style.getPropertyValue("height"));
  
  if (box_sizing === 'border-box') {
    const padding_top = parseInt(style.getPropertyValue("padding-top"));
    const padding_bottom = parseInt(style.getPropertyValue("padding-bottom"));
    const border_top = parseInt(style.getPropertyValue("border-top-width"));
    const border_bottom = parseInt(style.getPropertyValue("border-bottom-width"));
    height = height - padding_top - padding_bottom - border_top - border_bottom;
  }

  return height;
}

export const getLineHeight = (target: HTMLElement): number => {
    const style = getComputedStyle(target as Element, null);
    const line_height = parseInt(style.getPropertyValue("line-height"));
    const font_size = parseInt(style.getPropertyValue("font-size"));
    return (isNaN(line_height)) ? font_size * 1.2 : line_height;
}

export const countLines = (target: HTMLElement): number => {
  return Math.ceil(getContentHeight(target) / getLineHeight(target));
}
