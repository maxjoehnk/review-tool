import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/ffi.dart';
import 'package:review_tool/service_binder.dart';
import 'package:review_tool/settings.dart';

class NavBar extends StatefulWidget {
  final Widget settings;
  final WidgetBuilder view;

  const NavBar({required this.view, required this.settings, Key? key}) : super(key: key);

  @override
  State<NavBar> createState() => _NavBarState();
}

class _NavBarState extends State<NavBar> {
  bool settingsOpen = false;
  ProviderApi? _providerApi;

  @override
  Widget build(BuildContext context) {
    var settings = context.read<Settings>();
    return Column(children: [
      Container(
          decoration: BoxDecoration(
              color: Colors.grey.shade900,
              boxShadow: const [BoxShadow(spreadRadius: 2, blurRadius: 2, color: Colors.black45)]),
          padding: const EdgeInsets.symmetric(horizontal: 8),
          height: 40,
          child: Row(children: [
            ...settings.providers.map((provider) => TextButton(
                onPressed: () => setState(() {
                      settingsOpen = false;
                      _selectProvider(provider);
                    }),
                child: Text(provider.name))),
            const Spacer(),
            TextButton(
                onPressed: () => setState(() => settingsOpen = true),
                child: const Text("Settings")),
          ])),
      Expanded(child: _child)
    ]);
  }

  Widget get _child {
    if (settingsOpen) {
      return widget.settings;
    }
    if (_providerApi == null) {
      var settings = context.read<Settings>();
      return ListView(
          children: settings.providers
              .map((provider) =>
                  ListTile(title: Text(provider.name), onTap: () => _selectProvider(provider)))
              .toList());
    }
    return RepositoryProvider.value(
        value: _providerApi!,
        child: ProviderServiceBinder(_providerApi!, child: widget.view(context)));
  }

  _selectProvider(ProviderSettings provider) {
    setState(() => _providerApi = ProviderApi(api, provider.id));
  }
}
