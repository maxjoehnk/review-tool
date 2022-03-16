import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/ffi.dart';
import 'package:review_tool/state/review_list_state.dart';

import 'settings.dart';

class ServiceBinder extends StatelessWidget {
  final Widget child;
  final Settings settings;

  const ServiceBinder({required this.child, required this.settings, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MultiRepositoryProvider(providers: [
      RepositoryProvider<Native>.value(value: api),
      RepositoryProvider<Settings>.value(value: settings),
    ], child: child);
  }
}

class ProviderServiceBinder extends StatelessWidget {
  final ProviderApi providerApi;
  final Widget child;
  late final ReviewListState _reviewListState;

  ProviderServiceBinder(this.providerApi, {required this.child, Key? key}) : super(key: key) {
    _reviewListState = ReviewListState(providerApi);
  }

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(providers: [
      BlocProvider<ReviewListState>.value(value: _reviewListState),
    ], child: child);
  }
}
