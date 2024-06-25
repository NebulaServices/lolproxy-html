self.__lol$config = {
  prefix: "/service/",
  encodeUrl: function encode(str) {
    if (!str) return str;
    return encodeURIComponent(
      str
        .toString()
        .split("")
        .map((char, ind) =>
          ind % 2 ? String.fromCharCode(char.charCodeAt() ^ 3) : char
        )
        .join("")
    );
  },
  decodeUrl: function decode(str) {
    if (!str) return str;
    let [input, ...search] = str.split("?");

    return (
      decodeURIComponent(input)
        .split("")
        .map((char, ind) =>
          ind % 2 ? String.fromCharCode(char.charCodeAt(0) ^ 3) : char
        )
        .join("") + (search.length ? "?" + search.join("?") : "")
    );
  },
};
