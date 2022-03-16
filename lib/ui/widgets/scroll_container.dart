import 'package:flutter/material.dart';

typedef ScrollWidgetBuilder = Widget Function(BuildContext context, ScrollController controller);

class ScrollContainer extends StatefulWidget {
  final ScrollWidgetBuilder builder;

  const ScrollContainer({required this.builder, Key? key}) : super(key: key);

  @override
  State<ScrollContainer> createState() => _ScrollContainerState();
}

class _ScrollContainerState extends State<ScrollContainer> {
  final ScrollController _scrollController = ScrollController();

  @override
  Widget build(BuildContext context) {
    return Scrollbar(
      isAlwaysShown: true,
      trackVisibility: true,
      controller: _scrollController,
      child: ScrollConfiguration(
          behavior: ScrollConfiguration.of(context).copyWith(scrollbars: false),
          child: widget.builder(context, _scrollController)),
    );
  }
}
