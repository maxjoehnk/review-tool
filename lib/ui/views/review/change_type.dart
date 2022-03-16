import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';

class ReviewFileChangeType extends StatelessWidget {
  final ChangeType changeType;

  const ReviewFileChangeType({required this.changeType, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Tooltip(
        waitDuration: const Duration(seconds: 1),
        message: _changeMessage,
        child: Icon(_changeIcon, color: _changeIconColor, size: 16));
  }

  IconData get _changeIcon {
    switch (changeType) {
      case ChangeType.Added:
        return Icons.add;
      case ChangeType.Modified:
        return Icons.edit;
      case ChangeType.Removed:
        return Icons.remove;
    }
  }

  Color get _changeIconColor {
    switch (changeType) {
      case ChangeType.Added:
        return Colors.green;
      case ChangeType.Modified:
        return Colors.blue;
      case ChangeType.Removed:
        return Colors.red;
    }
  }

  String get _changeMessage {
    switch (changeType) {
      case ChangeType.Added:
        return "Added";
      case ChangeType.Modified:
        return "Modified";
      case ChangeType.Removed:
        return "Removed";
    }
  }
}
