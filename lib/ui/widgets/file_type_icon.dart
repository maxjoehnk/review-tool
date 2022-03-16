import 'package:dev_icons/dev_icons.dart';
import 'package:flutter/material.dart';

Map<String, IconData> icons = {
  "cs": DevIcons.csharpPlain,
  "csproj": DevIcons.dotnetPlain,
  "js": DevIcons.javascriptPlain,
  "jsx": DevIcons.reactOriginal,
  "ts": DevIcons.typescriptPlain,
  "tsx": DevIcons.reactOriginal,
  "css": DevIcons.css3Plain,
  "scss": DevIcons.sassOriginal,
  "dart": DevIcons.dartPlain,
  "go": DevIcons.goPlain,
  "rs": DevIcons.rustPlain,
  "php": DevIcons.phpPlain,
  "yarn.lock": DevIcons.yarnPlain,
  "package-lock.json": DevIcons.npmOriginalWordmark,
  "package.json": DevIcons.npmOriginalWordmark,
  "Dockerfile": DevIcons.dockerPlain,
  "png": Icons.image,
  "jpg": Icons.image,
  "jpeg": Icons.image,
};

IconData getFileTypeIcon(String filename) {
  for (var ending in icons.keys) {
    if (filename.endsWith(ending)) {
      return icons[ending]!;
    }
  }
  return Icons.text_snippet;
}

class FileType extends StatelessWidget {
  final String filename;

  const FileType({required this.filename, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var icon = getFileTypeIcon(filename);

    return Icon(icon, size: 16, color: Colors.white70);
  }
}
