import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/api.dart' hide ReviewState;
import 'package:review_tool/state/review_state.dart';
import 'package:review_tool/ui/widgets/file_type_icon.dart';
import 'package:review_tool/ui/widgets/scroll_container.dart';

import 'change_type.dart';

class ReviewFileTree extends StatelessWidget {
  const ReviewFileTree({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<ReviewCubit, ReviewState>(builder: (context, state) {
      if (state.loading) {
        return ListView.builder(
            itemBuilder: (context, i) => const LoadingReviewFileItem(), itemCount: 10);
      }
      var groupedChanges = groupBy<ReviewFileSummary, String>(
          state.files, (file) => file.filePathSegments.join("/"));
      List<Widget> widgets = [];
      groupedChanges.forEach((key, files) {
        widgets.add(ReviewFolderTreeItem(key));
        for (var file in files) {
          widgets.add(ReviewFileTreeItem(file));
        }
      });

      return ScrollContainer(
        builder: (context, scrollController) => Padding(
          padding: const EdgeInsets.only(right: 16.0),
          child: ListView(
            controller: scrollController,
            children: widgets,
          ),
        ),
      );
    });
  }
}

class ReviewFileTreeItem extends StatelessWidget {
  final ReviewFileSummary file;

  const ReviewFileTreeItem(this.file, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
            child: InkWell(
          mouseCursor: SystemMouseCursors.click,
          onTap: () => context.read<ReviewCubit>().selectFile(file.filePath, file.revisionId),
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 2.0),
            child: Row(children: [
              ReviewFileChangeType(changeType: file.changeType),
              const Padding(padding: EdgeInsets.all(4)),
              FileType(filename: file.fileName),
              const Padding(padding: EdgeInsets.all(4)),
              Expanded(
                child: Text(file.fileName,
                    overflow: TextOverflow.ellipsis,
                    style:
                        TextStyle(fontWeight: file.isRead ? FontWeight.normal : FontWeight.bold)),
              ),
            ]),
          ),
        )),
        const Padding(padding: EdgeInsets.all(4)),
        if (file.addedLines > 0)
          Text("+${file.addedLines}", style: const TextStyle(color: Colors.green)),
        if (file.removedLines > 0)
          Padding(
            padding: const EdgeInsets.only(left: 4.0),
            child: Text("-${file.removedLines}", style: const TextStyle(color: Colors.red)),
          ),
        const Padding(padding: EdgeInsets.all(4)),
        FileReadIndicator(file),
      ],
    );
  }
}

class ReviewFolderTreeItem extends StatelessWidget {
  final String path;

  const ReviewFolderTreeItem(this.path, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: Tooltip(
        message: path,
        waitDuration: const Duration(seconds: 1),
        child: Row(children: [
          const Icon(
            Icons.folder,
            color: Colors.white54,
            size: 16,
          ),
          const Padding(padding: EdgeInsets.all(4)),
          Expanded(
              child: Text(path,
                  style: const TextStyle(color: Colors.white54), overflow: TextOverflow.ellipsis)),
        ]),
      ),
    );
  }
}

class FileReadIndicator extends StatelessWidget {
  final ReviewFileSummary file;

  const FileReadIndicator(this.file, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return InkWell(
        mouseCursor: SystemMouseCursors.click,
        onTap: () => context.read<ReviewCubit>().toggleFileRead(file),
        child: Icon(file.isRead ? Icons.check_circle_outline : Icons.circle_outlined, size: 16));
  }
}

class LoadingReviewFileItem extends StatelessWidget {
  const LoadingReviewFileItem({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container();
  }
}
