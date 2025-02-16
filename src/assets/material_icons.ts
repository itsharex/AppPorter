export function generateMaterialIconsClasses() {
  let style = document.getElementById("material-icons-style");
  if (!style) {
    style = document.createElement("style");
    style.id = "material-icons-style";
    document.head.appendChild(style);
  }

  const processedIcons = new Set<string>();
  let updateScheduled = false;

  const updateStyles = () => {
    if (updateScheduled) return;
    updateScheduled = true;

    requestAnimationFrame(() => {
      const rules = Array.from(processedIcons)
        .map(
          (iconName) => `
          .mir.${iconName}::before {
            font-family: 'Material Symbols Rounded';
            content: "${iconName}";
            display: inline-block !important;
            font-style: normal;
            font-weight: normal !important;
            font-variation-settings: 'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24;
          }
        `,
        )
        .join("\n");

      style!.textContent = rules;
      updateScheduled = false;
    });
  };

  const processElement = (el: Element) => {
    const classList = el.classList ? Array.from(el.classList) : [];
    const mirIndex = classList.indexOf("mir");
    if (mirIndex !== -1 && mirIndex < classList.length - 1) {
      processedIcons.add(classList[mirIndex + 1]);
    }
  };

  document.querySelectorAll("*").forEach(processElement);
  updateStyles();

  const observer = new MutationObserver((mutations) => {
    let hasChanges = false;
    for (const mutation of mutations) {
      for (const node of mutation.addedNodes) {
        if (node instanceof Element) {
          processElement(node);
          node.querySelectorAll("*").forEach(processElement);
          hasChanges = true;
        }
      }
    }
    if (hasChanges) {
      updateStyles();
    }
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true,
  });

  return () => observer.disconnect();
}
