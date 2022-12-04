import 'package:http/browser_client.dart';
import 'package:http/http.dart';

class IMSPostClient extends BrowserClient {
  @override
  Future<StreamedResponse> send(BaseRequest request) {
    request.headers.clear();
    request.headers.putIfAbsent('Content-Type', () => 'application/json');
    return super.send(request);
  }
}
