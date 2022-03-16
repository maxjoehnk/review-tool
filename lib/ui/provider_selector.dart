import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/settings.dart';

class ProviderSelector extends StatefulWidget {
  final Widget child;

  const ProviderSelector({required this.child, Key? key}) : super(key: key);

  @override
  State<ProviderSelector> createState() => _ProviderSelectorState();
}

class _ProviderSelectorState extends State<ProviderSelector> {
  ProviderApi? _providerApi;

  @override
  Widget build(BuildContext context) {
    if (_providerApi != null) {
      return RepositoryProvider.value(value: _providerApi!, child: widget.child);
    }
    var settings = context.read<Settings>();
    var api = context.read<Native>();

    return ListView(
        children: settings.providers
            .map((provider) => ListTile(
                title: Text(provider.title),
                onTap: () => setState(() => _providerApi = ProviderApi(api, provider.id))))
            .toList());
  }
}
