import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_highlight/flutter_highlight.dart';
import 'package:flutter_highlight/themes/darcula.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/ui/widgets/file_type_icon.dart';
import 'package:review_tool/ui/widgets/scroll_container.dart';

import 'change_type.dart';

class ReviewFileView extends StatelessWidget {
  final Review review;
  final ReviewFileSummary file;

  const ReviewFileView(this.review, this.file, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textTheme = Theme.of(context).textTheme;
    return Card(
        child: Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
        Row(children: [
          ReviewFileChangeType(changeType: file.changeType),
          const Padding(padding: EdgeInsets.all(4)),
          FileType(filename: file.fileName),
          const Padding(padding: EdgeInsets.all(4)),
          Expanded(child: Text(file.fileName, style: textTheme.titleLarge)),
          if (file.addedLines > 0)
            Text("+${file.addedLines}", style: const TextStyle(color: Colors.green)),
          if (file.removedLines > 0)
            Padding(
              padding: const EdgeInsets.only(left: 4.0),
              child: Text("-${file.removedLines}", style: const TextStyle(color: Colors.red)),
            ),
          const Padding(padding: EdgeInsets.all(4)),
          OutlinedButton.icon(
              onPressed: () {},
              icon: const Icon(Icons.keyboard_arrow_up),
              label: const Text("Previous File")),
          const Padding(padding: EdgeInsets.all(2)),
          OutlinedButton.icon(
              onPressed: () {},
              icon: const Icon(Icons.keyboard_arrow_down),
              label: const Text("Next File")),
        ]),
        Text(file.filePathSegments.join("/"),
            style: const TextStyle(color: Colors.white54), overflow: TextOverflow.ellipsis),
        const Padding(padding: EdgeInsets.all(4)),
        Expanded(child: FileContent(review, file))
      ]),
    ));
  }
}

class FileContent extends StatelessWidget {
  final Review review;
  final ReviewFileSummary file;

  const FileContent(this.review, this.file, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    ProviderApi api = context.read();
    return FutureBuilder<ReviewFileChanges>(
      future: api.getReviewFile(
          reviewId: review.id, filePath: file.filePath, revision: file.revisionId),
      builder: (context, state) {
        if (!state.hasData) {
          log("loading or error $state");
          return Container();
        }

        return CodeViewer(file, state.data!);
      },
    );
  }
}

Map<String, String> languages = {
  "ts": "typescript",
  "tsx": "typescript",
  "tsx.snap": "typescript",
  "js": "javascript",
  "jsx": "javascript",
  "jsx.snap": "javascript",
  "cs": "cs",
  "csproj": "xml",
  "css": "css",
  "scss": "scss",
  "rs": "rust",
  "sql": "sql",
  "yml": "yaml",
  "yaml": "yaml",
  "Dockerfile": "dockerfile",
  "html": "xml",
  "xml": "xml",
  "sh": "bash",
  "json": "json",
  "Makefile": "makefile",
  "md": "markdown",
};

String getFileLanguage(String filename) {
  for (var ending in languages.keys) {
    if (filename.endsWith(ending)) {
      return languages[ending]!;
    }
  }
  return "plaintext";
}

class CodeViewer extends StatelessWidget {
  final ReviewFileSummary fileSummary;
  final ReviewFileChanges file;

  const CodeViewer(this.fileSummary, this.file, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var fileLanguage = getFileLanguage(fileSummary.fileName);
    return ScrollContainer(
      builder: (context, scrollController) => SingleChildScrollView(
          controller: scrollController,
          child: HighlightView(
            file.text,
            language: fileLanguage,
            padding: const EdgeInsets.all(8),
            theme: darculaTheme,
            tabSize: 4,
          )),
    );
  }
}
